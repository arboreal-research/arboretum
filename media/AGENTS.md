# Media Directory

**Project Assets and Images**

---

## 📑 Table of Contents

| Section | Description |
|---------|-------------|
| [Overview](#overview) | Media files purpose |
| [Assets](#assets) | Available images |
| [Usage](#usage) | How to use media files |
| [Adding New Media](#adding-new-media) | Guidelines for adding images |

---

## Overview

The `media/` directory contains images used in documentation and project assets.

## Assets

| File | Description | Size |
|------|-------------|------|
| `arboretum2.webp` | Project logo (decopunk style) | ~165 KB |
| `arboretum_original.webp` | Original Arboretum logo | ~560 KB |

## Usage in Documentation

These images are referenced in the main README.md:

```markdown
<p align="center">
  <img src="./media/arboretum2.webp" width="700" alt="An arboretum in decopunk style">
</p>
```

## Image Specifications

### arboretum2.webp
- **Format**: WebP (web-optimized)
- **Dimensions**: Approx 700px width
- **Purpose**: Project branding and documentation headers

### arboretum_original.webp
- **Format**: WebP (web-optimized)
- **Dimensions**: Original resolution
- **Purpose**: High-resolution version for backups

## Format Notes

- **WebP format** provides excellent compression while maintaining quality
- **Animated versions** are not used (static images only)
- Images are optimized for documentation rendering

## Adding New Media

When adding new images:

1. Use WebP format when possible for web efficiency
2. Resize to appropriate dimensions before adding
3. Update README.md with new image references
4. Keep file sizes reasonable (< 1MB for most images)

## Version Control

- Images should be committed to the repository
- No automated generation needed (static assets)
- Consider using `git lfs` for very large images if needed
