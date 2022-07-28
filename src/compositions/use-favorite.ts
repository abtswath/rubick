import { Ref } from 'vue';
import useRequest from './use-request';
import { Resource } from './use-resource';

export default (resource: Ref<Resource | null>) => {

    const { loading, request } = useRequest();

    const favorite = () => {
        if (resource.value === null) {
            return;
        }
        request('favorite', { resourceId: resource.value.id })
            .then(() => {
                if (resource.value) {
                    resource.value.favorite = true;
                }
            })
            .catch(() => { });
    }

    const unFavorite = () => {
        if (resource.value === null) {
            return;
        }
        request('favorite', { resourceId: resource.value.id })
            .then(() => {
                if (resource.value) {
                    resource.value.favorite = false;
                }
            })
            .catch(() => { });
    }

    return { favorite, unFavorite };

}
