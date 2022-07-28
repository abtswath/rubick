import convertSrc from '@/libs/convert-src';
import { ref } from 'vue';
import useRequest from './use-request';

export interface Resource {
    id: number;
    name: string;
    original_name: string;
    alias_name: string;
    pic: string;
}

export default () => {
    const { loading, request } = useRequest();

    const resources = ref<Resource[]>([]);

    request<Resource[]>('favorites')
        .then((response) => Promise.all(response.map(async item => {
            item.pic = await convertSrc(item.pic);
            return item;
        })))
        .then(data => {
            resources.value = data;
        })
        .catch(() => { });

    return {
        loading,
        resources
    }
}
