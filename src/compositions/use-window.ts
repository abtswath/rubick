import { appWindow, LogicalSize } from '@tauri-apps/api/window'
import { ref } from 'vue';

export default () => {
    const isMaximized = ref(false);
    const allowMaximized = ref(false);

    const setSize = async (width: number, height: number) => {
        const size = new LogicalSize(width, height);
        await appWindow.setMaxSize(size);
        await appWindow.setSize(size);
    };

    const show = async () => {
        await appWindow.show();
    }

    const maximize = async () => {
        await appWindow.maximize();
    };

    const minimize = async () => {
        await appWindow.minimize();
    }

    const unMaximize = async () => {
        await appWindow.unmaximize();
    }

    const toggleMaximize = async () => {
        if (!allowMaximized.value) {
            return;
        }
        await appWindow.toggleMaximize();
        isMaximized.value = await appWindow.isMaximized();
    }

    const close = async () => {
        await appWindow.close();
    }

    const setResizable = async (resizable: boolean) => {
        await appWindow.setResizable(resizable);
    }

    const disableMaximized = () => {
        allowMaximized.value = false;
    }

    const enableMaximized = () => {
        allowMaximized.value = true;
    }

    return {
        setSize,
        maximize,
        unMaximize,
        toggleMaximize,
        minimize,
        isMaximized,
        close,
        setResizable,
        allowMaximized,
        disableMaximized,
        enableMaximized,
        show
    }
}
