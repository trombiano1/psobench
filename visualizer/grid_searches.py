import pathlib
from pso import PSO
from tqdm import tqdm
from matplotlib.ticker import LogLocator, LogFormatterMathtext
import matplotlib.pyplot as plt  # type: ignore
from typing import Any
from grid_search import GridSearch
import matplotlib.image as mpimg

class GridSearches:
    def __init__(self, data: pathlib.Path, graphs: pathlib.Path, path: pathlib.Path):
        self.path = path
        self.data = data / path
        self.graphs = graphs / path

        if not self.data.exists():
            raise ValueError(f"Path {path} does not exist.")

        self.data_paths = []
        self.graph_paths = []
        for function_dir in self.data.glob("*"):
            self.data_paths.append(function_dir)
            self.graph_paths.append(self.graphs / function_dir.name)

        self.data_paths = sorted(self.data_paths, key=lambda path: path.name)
        self.graph_paths = sorted(self.graph_paths, key=lambda path: path.name)

    def draw_heatmap(self, log_1: bool, log_2: bool):
        for idx, function_dir in enumerate(self.data_paths):
            graph_dir = self.graphs / function_dir.name
            graph_dir.mkdir(parents=True, exist_ok=True)
            grid = GridSearch(function_dir)
            grid.draw_heatmap(graph_dir, log_1, log_2)

    def heatmap_collage(self, filename: str, log_1: bool, log_2: bool):
        self.draw_heatmap(log_1, log_2)
        n_rows = 5
        n_cols = 6

        fig, axs = plt.subplots(n_rows, n_cols, figsize=(10, 8), dpi=500)
        axs = axs.flatten()

        for i, ax in enumerate(axs):
            if i < len(self.graph_paths):
                img = mpimg.imread(self.graph_paths[i] / filename)
                ax.imshow(img)
            ax.axis('off')

        plt.subplots_adjust(wspace=0.1, hspace=0.1)
        print(f"Saving: {self.graphs / filename,}")
        plt.savefig(self.graphs / filename, bbox_inches='tight', pad_inches=0.1)

    def plot_best_global_progresses(self, axs) -> Any:
        axs = axs.flatten()
        for i, ax in enumerate(tqdm(axs)):
            if i >= len(self.data_paths):
                axs[i].set_visible(False)
                continue
            grid = GridSearch(self.data_paths[i])
            axs[i].yaxis.set_major_locator(LogLocator(base=10.0))
            axs[i].yaxis.set_major_formatter(LogFormatterMathtext(base=10.0, labelOnlyBase=False))
            axs[i].plot(grid.best_global_progress(), label=self.path.name)
            axs[i].set_title(grid.name)
            axs[i].legend()
        return axs
