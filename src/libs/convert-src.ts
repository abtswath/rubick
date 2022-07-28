import { appDir, join } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/tauri';

export default async (file: string) => {
    if (file === '') {
        return '';
    }
    const dir = await appDir();
    const path = await join(dir, 'images', file);
    return convertFileSrc(path);
}
