import { tauri } from "@tauri-apps/api";

export function setupFetchStatus(element: HTMLButtonElement) {
  element.innerHTML = `fetch steam status`;
  element.addEventListener('click', () => {
    // prints to javascript console; open dev tools for app with `⌥⌘I` (macos) or `⇧^I` (other)
    console.log("Hello, World!");
    // prints to stdout in the terminal
    tauri.invoke("fetch_status");
  })
}
