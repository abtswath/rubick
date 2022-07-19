import { ref } from 'vue';
import useHistory from './use-history';
import useRequest from './use-request';

export interface SearchResult {
    id: number,
    name: string,
    original_name: string,
    alias_name: string,
    channel: string,
}

export default () => {
    const { put } = useHistory();
    const { loading, request } = useRequest();
    const resources = ref<SearchResult[]>([]);

    const search = async (keyword: string) => {
        put(keyword);
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
