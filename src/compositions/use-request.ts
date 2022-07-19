import { ref } from 'vue';
import client from '@/libs/request';
import { useMessage } from 'naive-ui';
import { Response } from '@/libs/response';

export class RequestError extends Error {
    public code: number;

    constructor(message: string, code: number) {
        super(message);
        this.code = code;
    }
}

export default () => {
    const loading = ref(false);
    const message = useMessage();

    const request = <T>(url: string, data?: {}): Promise<T> => {
        return new Promise((resolve, reject) => {
            loading.value = true;
            client.request<Response<T>>(url, data)
                .then((response) => {
                    if (response.code !== 0) {
                        throw new RequestError(response.message, response.code);
                    }
                    resolve(response.data);
                })
                .catch(e => {
                    message.error(e.message || '未知错误');
                    reject(e);
                })
                .finally(() => {
                    loading.value = false;
                });
        });
    }

    return {
        loading,
        request
    };
}
