function initDashboardCharts(classes, assets) {
  if (typeof Chart === "undefined") return;

  const allocationCanvas = document.getElementById("allocationChart");
  const assetsCanvas = document.getElementById("assetsBarChart");
  const colors = ["#6366f1", "#10b981", "#38bdf8", "#f59e0b", "#a855f7", "#f43f5e", "#64748b"];

  if (allocationCanvas && classes.length) {
    new Chart(allocationCanvas, {
      type: "doughnut",
      data: {
        labels: classes.map((item) => item.label),
        datasets: [{ data: classes.map((item) => item.value), backgroundColor: colors }]
      },
      options: { responsive: true, maintainAspectRatio: false, plugins: { legend: { position: "bottom" } } }
    });
  }

  if (assetsCanvas && assets.length) {
    new Chart(assetsCanvas, {
      type: "bar",
      data: {
        labels: assets.map((item) => item.ticker),
        datasets: [{ label: "Valor atual (R$)", data: assets.map((item) => item.value), backgroundColor: "#6366f1" }]
      },
      options: { responsive: true, maintainAspectRatio: false, scales: { y: { beginAtZero: true } } }
    });
  }
}
