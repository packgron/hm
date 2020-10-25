# Home Manager (hm)

## Concept

 A home management tool, to sync home directoty described in declarative way.

 Home directory contains dotfiles/private dir/described home directories.


### File structure

```
home
+-- .hm
|   +-- sync
|   |   +-- .git        // Git backup project used for syncing
|   |   +-- .dotfile
|   |   +-- directory
|   |   +-- config.yaml // Home config, link/clone info (selfmodifying)
|   +-- crypt           // gocryptfs mountpoints
|       +-- private
|
+-- .dotfile  -> .hm/sync/.dotfile
+-- private   -> crypt://(vcs://private or .hm/somecryptdir)
+-- directory -> .hm/sync/directory
+-- vcs_dir   -> vcs://dir
+-- work
|   +-- nest  -> vcs://nest
+-- hanging
```

### Config

```yaml
version: 1

deps:
  - package

sync:
  .dotfile: .dotfile
  directory: directory
  vcs_dir: vcs://dir
  private: crypt://vcs://private
```


### Debs

 Dependence are loaded in distro/os independed ways.
 Package mappings are described in share/hm/deps.yaml.
 Main pakage names are identical to Arch pacman packages.
 deps.yaml:
```yaml
 debian:
   devel: build-essensials

```


### CLI

```
// item - inhome:insync / insync = plain | protocol://(insync)
$ hm items a[dd] <item> [<item>...]         // Add item to be synced
$ hm items r[emove] <item> [<item>...]      // Remove item from syncing (cp to home, remove from config and sync dir)
$ hm items ls [-s|--synced] [-l|--local]    // List all synced files or all already installed items
$ hm items diff                             // Show diff of items to be added

$ hm deps i[nstall] [<deb>...]              // Install all deps or specific deps
$ hm deps u[pdate] [<deb>...]               // Update all deps or specific deps
$ hm deps r[emove] [<deb>...]               // Remove all deps or specific deps
$ hm deps ls [-s|--synced] [-l|--local]     // List synced deps or all local deps (only mapped)
$ hm deps diff                              // Show diff of items need to be added

$ hm clean                                  // Clean hm synced mappings (mapping links
$ hm sync                                   // Sync all the thins (flags for hard/soft ?)
$ hm diff                                   // Diff of all fings needed (flags for hard/soft ?)
$ hm start [-p <pass>]                      // Start home managent session (begin new diff items/deps, setup password for crypto)
$ hm stop                                   // End session, clear pass, sync direcoties to remotes.
```


## TODO:

[] config.yaml parsing:
    [] version parsing
    [] sync parsing:
        [] basic linking config
        [] git:// parsing
        [] crypt:// parsing with nested vcs or link parsing
    [] deps parsing

[] context:
    [] version + check
    [] sync model
       [] linking list
       [] git pull list
       [] crypt dir list
    [] deb pakage mapping hash table

[] mechanizims:
    [] model from dir
    [] config/dir_model diff
    [] linking calls
    [] git clone calls
    [] crypt:
        [] password store
        [] crypt dir population
        [] mounting
    [] new sync add
    [] diff:
        [] main directory comparison
        [] git directory diffs
        [] crypto directory diffs
    [] sync:
        [] push main directory
        [] push sub git directories
        [] push crypt dirs
    [] packages:
        [] current pakages diff (not in config)
        [] package installion
    [] session start/end


