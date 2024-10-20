import { invoke } from "@tauri-apps/api/core";
import { platform, version } from '@tauri-apps/plugin-os';
import { getAllWindows } from "@tauri-apps/api/window";

export enum Theme {
  Auto = "auto",
  Light = "light",
  Dark = "dark",
}

export async function setTheme(theme: Theme) {
  await invoke("plugin:appearance|cmd_set_theme", {
    theme,
  });
  const p = await platform();
  if (p === 'windows') {
    const v = await version();
    if (v < '10.0.22000') {
      for(const w of await getAllWindows()) {
        const isMaximized = await w.isMaximized();
        if (isMaximized) {
          await w.unmaximize();
          await w.maximize();
        } else {
          const size = await w.innerSize();
          size.width += 1;
          await w.setSize(size);
          size.width -= 1;
          await w.setSize(size);
        }
      }
    }
  }
};

export async function getTheme() {
  const theme = await invoke("plugin:appearance|cmd_get_theme");
  return theme as Theme;
};
