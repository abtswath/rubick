import { appDir, join } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/tauri';

const dir = await appDir();

export default async (file: string) => {
    if (file === '') {
        return '';
    }
    const path = await join(dir, 'images', file);
    return convertFileSrc(path);
}
