import { getTheme, setTheme, Theme } from "@kuyoonjo/tauri-plugin-theme-v2";

document.getElementById("auto")!.onclick = () => setTheme(Theme.Auto);
document.getElementById("light")!.onclick = () => setTheme(Theme.Light);
document.getElementById("dark")!.onclick = () => setTheme(Theme.Dark);

document.getElementById("get")!.onclick = async () => {
  const theme = await getTheme();
  alert(`Current theme value: '${theme}'`);
};
