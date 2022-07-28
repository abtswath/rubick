import { ref } from 'vue';
import { useRouter } from 'vue-router';

export default () => {
    const keepalivePages = ref<string[]>([]);

    const router = useRouter();

    router.beforeEach((to, from, next) => {
        if (to.name === 'resource') {
            if (from.name !== null && from.name !== undefined) {
                keepalivePages.value = [from.name.toString()];
            }
        } else if (from.name !== 'resource') {
            keepalivePages.value = [];
        }
        next();
    });

    return keepalivePages;
}
