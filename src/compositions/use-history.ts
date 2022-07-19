import { ref } from 'vue';

const KEY = 'search-history';

export default () => {

    let localHistories: string[] = [];
    try {
        localHistories = JSON.parse(localStorage.getItem(KEY) || "[]");
    } catch (error) {
    }

    const histories = ref(localHistories);

    const put = (item: string) => {
        if (has(item)) {
            remove(item);
        }
        histories.value = [item, ...histories.value];
        localStorage.setItem(KEY, JSON.stringify(histories.value));
    }

    const indexOf = (item: string): number => {
        return histories.value.indexOf(item);
    }

    const has = (item: string): boolean => {
        return histories.value.indexOf(item) > -1;
    }

    const remove = (item: string) => {
        const index = indexOf(item);
        index > -1 && histories.value.splice(index, 1);
    }

    const clear = () => {
        histories.value = [];
        localStorage.removeItem(KEY);
    }

    return {
        histories,
        clear,
        put,
        indexOf,
        has,
        remove
    };
}
