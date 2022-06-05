import * as generator from 'generator';

// Constants
const defaultCellSize = 50;

// get elements from DOM
const form = document.getElementById('maze-options') as HTMLFormElement;
const mazeType = document.getElementById('maze-type') as HTMLSelectElement;
const mazeWidth = document.getElementById('maze-width') as HTMLInputElement;
const mazeHeight = document.getElementById('maze-height') as HTMLInputElement;
const mazeContainer = document.getElementById('maze') as HTMLDivElement;

form.addEventListener('submit', (event) => {
    event.preventDefault();

    let maze;

    switch ((mazeType.children[mazeType.selectedIndex] as HTMLOptionElement).value) {
        case 'rectangle':
            const width = mazeWidth.valueAsNumber;
            const height = mazeHeight.valueAsNumber;

            const start = [width / 2, 0];
            const end = [width / 2, height - 1];

            const size = Uint32Array.of(width, height, end[0]!, end[1]!, start[0]!, start[1]!)

            maze = generator.generate("rectangle", size, defaultCellSize);
            break;

        default:
            throw new Error('Unknown maze type');
    }

    console.log(maze);

    mazeContainer.innerHTML = maze;
})