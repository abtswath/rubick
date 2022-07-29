import { InjectionKey, State } from 'vue';
import { createStore, createLogger, Store, useStore as baseUseStore } from 'vuex';
import window from './modules/window';
import searchHistory from './modules/search-history';

export default createStore<State>({
    modules: {
        window,
        searchHistory
    },
    strict: !import.meta.env.PROD,
    plugins: import.meta.env.DEV ? [createLogger()] : []
});

export const key: InjectionKey<Store<State>> = Symbol();

export const useStore = () => {
    return baseUseStore(key);
}
