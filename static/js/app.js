(() => {
  const root = document.documentElement;
  const themeButton = document.getElementById("themeToggle");
  const menuButton = document.getElementById("sidebarToggle");

  const savedTheme = localStorage.getItem("carteira-theme");
  if (savedTheme) root.dataset.theme = savedTheme;

  themeButton?.addEventListener("click", () => {
    const nextTheme = root.dataset.theme === "light" ? "dark" : "light";
    root.dataset.theme = nextTheme;
    localStorage.setItem("carteira-theme", nextTheme);
  });

  menuButton?.addEventListener("click", () => {
    document.querySelector(".sidebar")?.classList.toggle("is-open");
  });
})();
