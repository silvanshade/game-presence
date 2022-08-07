function colorMode(): "light-mode" | "dark-mode" {
  if (window.matchMedia("(prefers-color-scheme: dark)")) {
    return "dark-mode";
  } else {
    return "light-mode";
  }
}

export { colorMode };
