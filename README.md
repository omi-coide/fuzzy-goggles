现在问题:sixel 显示后不会被覆盖,重点逻辑在wezterm的
wezterm-gui/src/termwindow/render/screen_line.rs
https://github.com/wez/wezterm/blob/11dec45f08e3c7a611f59b030d4a9bd391807604/wezterm-gui/src/termwindow/render/mod.rs#L412
https://github.com/wez/wezterm/blob/11dec45f08e3c7a611f59b030d4a9bd391807604/term/src/terminalstate/sixel.rs#L132

如果在这里更改z-index,是否会有效
https://github.com/wez/wezterm/blob/11dec45f08e3c7a611f59b030d4a9bd391807604/term/src/terminalstate/image.rs#L190C75-L190C75

UPDATE修改这里更合适
https://github.com/wez/wezterm/blob/11dec45f08e3c7a611f59b030d4a9bd391807604/term/src/terminalstate/sixel.rs#L132
