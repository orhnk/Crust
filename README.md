# Crust
Crust is lightweight incredibly fast text editor fully created with rust.

**Crust uses**;
<p>
  ∫ css for configuration,
  
  ∫ gtk4 as guilib,
  
  ∫ rust as language,
</p>


**Crust features**:
<p>
  ∫ No buttons! -> no need to click, use your keyboard instead.
  
  ∫ Simple! -> You could use simple combinations like: shift+ctrl+arrow keys to highlight a text by words
  
  ∫ Easy to configure -> Uses default gtk4 css cınfiguration which makes it super easy!
  
  ∫ Smooth animations -> Crust uses default gtk4 animations on a Text instance, which is pretty smooth and good-looking!
  
</p>

# Installation

**Cargo (Cross-platform)**
```
cargo install crust_ide
./install.sh
```
-> install.sh creates config files which allows you to costumize the text editor on your home (crust.css, crust.ui)

**Test:**

type `crust` anywhere on your terminal and something like this has to come out!



**Trouble Shooting:**

If your `crust` does not work on your terminal try this: (your .cargo/bin has to be on $PATH)
```rust
export PATH=$PATH:~/.cargo/bin/
```
```
cargo install crust_ide --force
```

**IMPORTANT**

You MUST have `crust.ui` and `crust.css` files on your `/usr/share/crust/` to be able to  use `crust command`

# IMPORTANT!

**Crust saves your files when you quit! There is no option to save otherwise for now. So do not delete some files and exit thinking that your files will going to be "safe" after unsaved quit! Crust saves are handled automatically!**

# Config

```css
/* This file is a config file which will determine your crust-editor's look */
/* This file uses real css to configure crust */

textview.view {
  /* Default gruvbox theme*/
  color: #ebdbb2;
  background-color: #282828;
  font-family: serif; /* -> ADDED <- */
}
```

![Screenshot_20221217_222102](https://user-images.githubusercontent.com/101834410/208262652-0dc769fa-9dc5-4907-928c-6437f9afbc01.png)

# Examples

![Screenshot_20221217_220507](https://user-images.githubusercontent.com/101834410/208259620-a77d58b4-eec6-450d-ab3a-6ce9f58e5af9.png)

**My personal config (Gruvbox Theme)**

![Screenshot_20221217_220639](https://user-images.githubusercontent.com/101834410/208260409-dc2a4ffb-f04b-4e50-b5b1-04aceec0642b.png)

**Saves When You quit!**

![Screenshot_20221217_220710](https://user-images.githubusercontent.com/101834410/208259697-b9ed7446-cd44-463c-9bae-86b85677c59d.png)

# Benchmarks

**extremely fast gui!**

```
./crust  0,22s user 0,14s system 111% cpu 0,330 total
./crust  0,26s user 0,14s system 116% cpu 0,344 total
./crust  0,25s user 0,10s system 119% cpu 0,285 total
./crust  0,23s user 0,14s system 111% cpu 0,336 total
./crust  0,25s user 0,12s system 115% cpu 0,322 total
./crust  0,24s user 0,14s system 110% cpu 0,343 total
```

