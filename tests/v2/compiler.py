#!/usr/bin/env python3
"""C++ compilation with Arboretum plugin."""

import subprocess
import tempfile
import os
from pathlib import Path
from typing import List, Optional, Dict, Any


class CompilationError(Exception):
    """Compilation failed."""
    def __init__(self, source_file: str, stdout: str, stderr: str):
        self.source_file = source_file
        self.stdout = stdout
        self.stderr = stderr
        super().__init__(f"Compilation failed for {source_file}")


class TestCompiler:
    """Compiles test files with Arboretum plugin."""
    
    def __init__(
        self,
        plugin_path: str,
        postgres_url: str,
        build_dir: Optional[str] = None,
        extra_args: Optional[List[str]] = None,
    ):
        """Initialize the compiler.
        
        Args:
            plugin_path: Path to libarboretum.so
            postgres_url: PostgreSQL connection URL
            build_dir: Output directory for compiled files (optional)
            extra_args: Extra arguments to pass to clang++
        """
        self.plugin_path = Path(plugin_path).resolve()
        self.postgres_url = postgres_url
        self.build_dir = Path(build_dir) if build_dir else Path(tempfile.mkdtemp())
        self.build_dir.mkdir(parents=True, exist_ok=True)
        self.extra_args = extra_args or []
        
        # Verify plugin exists
        if not self.plugin_path.exists():
            raise FileNotFoundError(f"Plugin not found: {self.plugin_path}")
    
    def compile(
        self,
        source_file: str,
        output_file: Optional[str] = None,
        cxx_std: str = "c++20",
    ) -> str:
        """Compile a source file with the Arboretum plugin.
        
        Args:
            source_file: Path to C++ source file
            output_file: Output file path (optional, auto-generated if not provided)
            cxx_std: C++ standard to use (default: c++20)
            
        Returns:
            Path to output file
        """
        source_path = Path(source_file)
        if not source_path.exists():
            raise FileNotFoundError(f"Source file not found: {source_file}")
        
        # Generate output file name if not provided
        if output_file is None:
            output_file = str(self.build_dir / source_path.stem)
        
        # Build compiler arguments
        args = [
            "clang++",
            "-c",
            f"-fplugin={str(self.plugin_path)}",
            f"-std={cxx_std}",
            "-fno-rtti",
            f"-fplugin-arg=arboretum-connect={self.postgres_url}",
            "-o", output_file,
            str(source_path),
        ]
        
        # Add extra arguments
        args.extend(self.extra_args)
        
        # Run compilation
        print(f"Compiling: {source_file}")
        result = subprocess.run(
            args,
            capture_output=True,
            text=True
        )
        
        if result.returncode != 0:
            raise CompilationError(
                source_file=source_file,
                stdout=result.stdout,
                stderr=result.stderr,
            )
        
        print(f"Compilation successful: {output_file}")
        return output_file
    
    def compile_multiple(
        self,
        source_files: List[str],
        output_dir: Optional[str] = None,
        cxx_std: str = "c++20",
    ) -> List[str]:
        """Compile multiple source files.
        
        Args:
            source_files: List of paths to C++ source files
            output_dir: Output directory (optional)
            cxx_std: C++ standard to use
            
        Returns:
            List of paths to output files
        """
        outputs = []
        for source_file in source_files:
            output = self.compile(source_file, output_dir, cxx_std)
            outputs.append(output)
        return outputs
    
    def compile_with_plugin_args(
        self,
        source_file: str,
        plugin_args: Dict[str, str],
        **kwargs,
    ) -> str:
        """Compile with custom plugin arguments.
        
        Args:
            source_file: Path to C++ source file
            plugin_args: Dictionary of plugin argument name -> value
            **kwargs: Additional arguments for compile()
            
        Returns:
            Path to output file
        """
        # Build plugin arguments
        extra_args = []
        for name, value in plugin_args.items():
            extra_args.append(f"-fplugin-arg=arboretum-{name}={value}")
        
        # Create new compiler with extra args
        compiler = TestCompiler(
            plugin_path=str(self.plugin_path),
            postgres_url=self.postgres_url,
            build_dir=str(self.build_dir),
            extra_args=self.extra_args + extra_args,
        )
        
        return compiler.compile(source_file, **kwargs)
    
    def clean_build_dir(self) -> None:
        """Clean up the build directory."""
        import shutil
        if self.build_dir.exists():
            shutil.rmtree(self.build_dir)
            self.build_dir.mkdir(parents=True, exist_ok=True)


def compile_test_files(
    source_files: List[str],
    plugin_path: str,
    postgres_url: str,
    output_dir: Optional[str] = None,
) -> List[str]:
    """Helper function to compile test files.
    
    Args:
        source_files: List of source file paths
        plugin_path: Path to libarboretum.so
        postgres_url: PostgreSQL connection URL
        output_dir: Output directory (optional)
        
    Returns:
        List of output file paths
    """
    compiler = TestCompiler(
        plugin_path=plugin_path,
        postgres_url=postgres_url,
        build_dir=output_dir,
    )
    
    return compiler.compile_multiple(source_files)


def verify_compilation(
    source_file: str,
    plugin_path: str,
    postgres_url: str,
    expected_tables: List[str] = None,
    db: Any = None,
) -> bool:
    """Compile and verify the results.
    
    Args:
        source_file: Source file to compile
        plugin_path: Path to libarboretum.so
        postgres_url: PostgreSQL connection URL
        expected_tables: List of expected table names
        db: TestDatabase instance (optional)
        
    Returns:
        True if compilation and verification succeeded
    """
    compiler = TestCompiler(
        plugin_path=plugin_path,
        postgres_url=postgres_url,
    )
    
    try:
        compiler.compile(source_file)
        
        if db and expected_tables:
            from .postgres import verify_tables_exist
            success, missing = verify_tables_exist(db, expected_tables)
            if missing:
                print(f"Missing tables: {missing}")
                return False
        
        return True
    except CompilationError as e:
        print(f"Compilation failed: {e.stderr}")
        return False
