import { useStore } from '@/store';
import { ref } from 'vue';
import useRequest from './use-request';

export interface SearchResult {
    id: number,
    name: string,
    original_name: string,
    alias_name: string,
    channel: string,
}

export default () => {
    const store = useStore();
    const { loading, request } = useRequest();
    const resources = ref<SearchResult[]>([]);

    const search = async (keyword: string) => {
        store.dispatch('searchHistory/put', keyword);
        await request<SearchResult[]>('search', { keyword: keyword })
            .then((response) => {
                resources.value = response;
            })
            .catch(() => { });
    }

    return {
        loading,
        search,
        resources
    };
}
