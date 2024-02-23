# rust-lilypond-invoke-editor
lilypond-invoke-editor made using Rust.

## Description

This software is designed to replace the lilypond-invoke-editor in Windows 10.

1. Build the code yourself with `cargo build --release`. Or get a prebuilt binary in [releases](https://github.com/Apostolique/rust-lilypond-invoke-editor/releases).
   - Put it in `C:\`. You will need administrator permissions.
2. In order to use this, first open `Registry Editor` (`regedit`), navigate to `Computer\HKEY_CLASSES_ROOT\textedit\shell\open\command`. You should see key named `(Default)`. It's type should be `REG_SZ`.
   - Set the data to `C:\rust-lilypond-invoke-editor.exe "%1"`.
   - Create a new `String Value` in `Computer\HKEY_CLASSES_ROOT\textedit`, name it `URL Protocol`.
3. To configure your text editor, edit your user environment variables.
   - Create a new variable called `LYEDITOR` if you don't already have one.
   - Set the value to `code -g %(file)s:%(line)s:%(column)s` if you use vscode.
4. To configure [SumatraPDF](https://github.com/sumatrapdfreader/sumatrapdf), edit `C:\Program Files\SumatraPDF\sumatrapdfrestrict.ini`. Find the line for `LinkProtocols`.
   - Append `,textedit` to that line. In my case, the line becomes:
   ```ini
   LinkProtocols = http,https,mailto,textedit
   ```
