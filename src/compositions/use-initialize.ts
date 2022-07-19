import { useDialog } from 'naive-ui';
import { Ref, ref } from 'vue';
import { useRouter } from 'vue-router';
import useWindow from './use-window';
import { event } from '@tauri-apps/api';
import { Response } from '@/libs/response';

type Step = 'downloading' | 'importing' | 'finish';

interface ResponseData {
    step: Step,
    data: any
}

export default (): [Ref<number>, Ref<number>, Ref<string>, Ref<Step>] => {
    const percentage = ref(0);
    const importPercentage = ref(0);
    const message = ref('');
    const dialog = useDialog();
    const { close } = useWindow();
    const step = ref<Step>('downloading');
    const router = useRouter();

    event.listen<Response<ResponseData>>('rubick_initialize', (event) => {
        if (event.payload.code !== 0) {
            dialog.error({
                content: event.payload.message,
                maskClosable: false,
                positiveText: '关闭',
                closable: false,
                closeOnEsc: false,
                showIcon: false,
                onPositiveClick: () => {
                    close();
                }
            });
            return;
        }
        message.value = event.payload.message;
        step.value = event.payload.data.step;
        if (event.payload.data.step === 'finish') {
            router.push({
                name: 'search'
            });
        } else if (event.payload.data.step === 'downloading') {
            const [processed, total] = event.payload.data.data as [number, number];
            importPercentage.value = Math.round(processed / total * 100);
        } else {
            const [processed, total] = event.payload.data.data as [number, number];
            percentage.value = Math.round(processed / total * 100);
        }
    });

    return [percentage, importPercentage, message, step];
}
