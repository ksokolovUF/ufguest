# ufguest

This program makes usage of ufguest less annoying by clicking the accept button for
you.

## What's wrong with eduroam

As you know, eduroam wifi maliciously forces you to install its CA.
By doing this it gains ability to perform a man-in-the-middle attac and decrypt ALL
your https traffic.
YES, IT SEES EVERYTHING: YOUR PASSWORDS, YOUR MESSAGES, YOUR FINANCE, YOUR SECRETS.
And they purposefully made ufguest disconnect mutiple times a day and super slow
so that you give up and switch to eduroam.

## Usage

Install [geckodriver](https://github.com/mozilla/geckodriver/releases).
> **_NOTE:_**  you can use `selenium-manager` to get geckodriver:
`sudo dnf install selenium-manager` and then `selenium-manager --browser firefox`

```bash
cd ufguest
cargo build --release
mkdir -p ~/bin
cp target/release/ufguest ~/bin
~/bin/ufguest ~/.cache/selenium/geckodriver/linux-arm64/0.36.0/geckodriver
```

You could also put a keybinding in your WM (Hyprland in this case):

```hyprlang
bind = SUPER_SHIFT, U, exec, ~/bin/ufguest "~/.cache/selenium/geckodriver/linux-arm64/0.36.0/geckodriver"
```
