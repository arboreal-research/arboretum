LLVM_DIR := $(shell realpath llvm/)
BUILD_DIR := $(shell realpath build/)
CXXFLAGS := -Wpedantic -Werror -std=c++20 -fno-rtti -isystem llvm/include/
LLVM_STAMP := llvm-project/build/llvm-stamp

REIFICATOR_SRCS := $(wildcard reificator/src/*.cc)
REIFY_CPP_SRCS := $(wildcard reify-cpp/src/*.cc)
ARBORETUM_SRCS := $(wildcard arboretum-plugin/src/*.cc)
ARBORETUM_FFI_SRCS := $(wildcard arboretum-ffi/src/*.rs)

REIFICATOR_OBJS := $(patsubst %.cc,$(BUILD_DIR)/reificator/%.o,$(notdir $(REIFICATOR_SRCS)))
REIFY_CPP_OBJS := $(patsubst %.cc,$(BUILD_DIR)/reify-cpp/%.o,$(notdir $(REIFY_CPP_SRCS)))
ARBORETUM_OBJS := $(patsubst %.cc,$(BUILD_DIR)/arboretum-plugin/%.o,$(notdir $(ARBORETUM_SRCS)))

arboretum: $(BUILD_DIR)/libarboretum.so

$(LLVM_STAMP):
	cd llvm-project \
		&& cmake -S llvm \
			-B build \
			-DLLVM_ENABLE_PROJECTS=clang \
			-DCMAKE_BUILD_TYPE=Release \
			-DLLVM_PARALLEL_LINK_JOBS=1 \
		&& cmake --build build -j12 \
		&& cmake --install build --prefix ${LLVM_DIR} \
		&& touch build/llvm-stamp

# REIFICATOR
reificator/properties.csv:

$(BUILD_DIR)/libreificator.so: $(REIFICATOR_OBJS)
	./llvm/bin/clang++ -shared -fPIC $(CXXFLAGS) -o $@ $^

$(BUILD_DIR)/reificator/%.o: reificator/src/%.cc | $(BUILD_DIR) $(LLVM_STAMP) reificator/properties.csv
	./llvm/bin/clang++ -g -c $(CXXFLAGS) -fPIC -Ireificator/include/ -MMD -MF $(BUILD_DIR)/reificator/$*.d -o $@ $< 

$(BUILD_DIR)/reificator-stamp: $(BUILD_DIR)/libreificator.so reificator/input.cc $(LLVM_STAMP) reificator/properties.csv
	./llvm/bin/clang++ -fplugin=$(BUILD_DIR)/libreificator.so -c $(CXXFLAGS) reificator/input.cc
	touch $(BUILD_DIR)/reificator-stamp

# REIFICATOR GENERATED CODE
$(BUILD_DIR)/reify-cpp.a: $(REIFY_CPP_OBJS) | $(BUILD_DIR)/reificator-stamp
	ar rcs $@ $^

$(BUILD_DIR)/reify-cpp/%.o: reify-cpp/src/%.cc | $(BUILD_DIR) $(LLVM_STAMP) $(BUILD_DIR)/reificator-stamp
	./llvm/bin/clang++ -g -c $(CXXFLAGS) -fPIC -Ireify-cpp/include/ -Iarboretum-ffi/src/ -MMD -MF $(BUILD_DIR)/reify-cpp/$*.d -o $@ $< 

# ARBORETUM FFI
arboretum-ffi/src/lib.rs: arboretum-ffi/src/tcp_client.rs 
arboretum-ffi/src/tcp_client.rs: 

target/release/libarboretum_ffi.a: arboretum-ffi/src/lib.rs arboretum-ffi/src/tcp_client.rs
	cd arboretum-ffi && cargo build --release

# # ARBORETUM CLANG PLUGIN
$(BUILD_DIR)/libarboretum.so: target/release/libarboretum_ffi.a $(BUILD_DIR)/reify-cpp.a $(ARBORETUM_OBJS)
	./llvm/bin/clang++ -shared -fPIC $(CXXFLAGS) -o $@ $(ARBORETUM_OBJS) $(BUILD_DIR)/reify-cpp.a target/release/libarboretum_ffi.a

$(BUILD_DIR)/arboretum-plugin/%.o: arboretum-plugin/src/%.cc | $(BUILD_DIR) $(LLVM_STAMP) $(BUILD_DIR)/reificator-stamp
	./llvm/bin/clang++ -g -c $(CXXFLAGS) -fPIC -Iarboretum-ffi/src/ -Ireify-cpp/include/ -MMD -MF $(BUILD_DIR)/arboretum-plugin/$*.d -o $@ $< 

$(BUILD_DIR):
	@mkdir -p $(BUILD_DIR)
	@mkdir -p $(BUILD_DIR)/reificator
	@mkdir -p $(BUILD_DIR)/reify-cpp
	@mkdir -p $(BUILD_DIR)/arboretum-plugin

# Include dependency files
-include $(REIFICATOR_OBJS:.o=.d)
-include $(REIFY_CPP_OBJS:.o=.d)
-include $(ARBORETUM_OBJS:.o=.d)

clean:
	rm -rf $(BUILD_DIR) target/release/libarboretum_ffi.a
