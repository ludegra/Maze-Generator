import * as generator from 'generator';

// Constants
const defaultCellSize = 50;

// get elements from DOM
const form = document.getElementById('maze-options') as HTMLFormElement;
const mazeType = document.getElementById('maze-type') as HTMLSelectElement;
const mazeWidth = document.getElementById('maze-width') as HTMLInputElement;
const mazeHeight = document.getElementById('maze-height') as HTMLInputElement;
const mazeRadius = document.getElementById('maze-radius') as HTMLInputElement;
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

            const rectSize = Uint32Array.of(width, height, end[0]!, end[1]!, start[0]!, start[1]!)

            maze = generator.generate("rectangle", rectSize, defaultCellSize);
            break;

        case 'circle':
            const radius = mazeRadius.valueAsNumber;
            const intensity = 2;

            const circleSize = Uint32Array.of(radius, intensity);

            maze = generator.generate("circle", circleSize, defaultCellSize);
            break;

        default:
            throw new Error('Unknown maze type');
    }

    mazeContainer.innerHTML = maze;
})

let old_type = (mazeType.children[mazeType.selectedIndex] as HTMLOptionElement).value;

mazeType.addEventListener('change', (event) => {
    const type = (event.target as HTMLSelectElement).value;

    if (type === old_type) return;

    const oldOptions = document.getElementById(`${old_type}-options`) as HTMLDivElement | null;
    const newOptions = document.getElementById(`${type}-options`) as HTMLDivElement | null;

    if (!(oldOptions && newOptions)) return;

    oldOptions.classList.add('hidden');
    newOptions.classList.remove('hidden');

    old_type = type;
})