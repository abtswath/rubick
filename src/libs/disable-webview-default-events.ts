const disableDefaultEvents = () => {
    const disableKeys = ['p', 'f', 'r', 'a'];

    const keydownHandler = (e: KeyboardEvent) => {
        if (e.key === 'F5' || (e.ctrlKey && disableKeys.indexOf(e.key) > -1)) {
            e.preventDefault();
        }
    }

    const clickHandler = (e: MouseEvent) => {
        if (e.shiftKey || e.ctrlKey || e.altKey) {
            e.preventDefault();
        }
    }

    const contextMenuHandler = (e: MouseEvent) => {
        e.preventDefault();
    }

    document.addEventListener('keydown', keydownHandler);
    document.addEventListener('click', clickHandler);
    document.addEventListener('contextmenu', contextMenuHandler);

    window.addEventListener('beforeunload', () => {
        document.addEventListener('keydown', keydownHandler);
        document.addEventListener('click', clickHandler);
        document.addEventListener('contextmenu', contextMenuHandler);
    });
}

export default disableDefaultEvents;
