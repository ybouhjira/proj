# proj Manual Page

## Install

To install the man page system-wide:

```bash
# Install to system man directory
sudo mkdir -p /usr/local/share/man/man1
sudo cp man/proj.1 /usr/local/share/man/man1/
sudo mandb

# Or install to user man directory (no sudo required)
mkdir -p ~/.local/share/man/man1
cp man/proj.1 ~/.local/share/man/man1/
mandb
```

## View

```bash
# View the man page
man proj

# View locally without installing
man -l man/proj.1
```

## Verify

```bash
# Check man page syntax
man --warnings -l man/proj.1

# Check for broken references
lexgrog man/proj.1
```

## Update

After editing `proj.1`, rebuild the man database:

```bash
sudo mandb
```
