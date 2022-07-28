const disabledCtrlKeys = ['p', 'f', 'r'];
const disabledKeys = ['F3', 'F7'];

const disableDefaultEvents = () => {

    const keydownHandler = (e: KeyboardEvent) => {
        if (disabledKeys.indexOf(e.key) > -1 || (e.ctrlKey && disabledCtrlKeys.indexOf(e.key) > -1)) {
            e.preventDefault();
        }
        const tagName = (e.currentTarget as HTMLElement).tagName?.toLowerCase();
        if (e.ctrlKey && e.key === 'a' && ['input', 'textarea'].indexOf(tagName) < 0) {
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
