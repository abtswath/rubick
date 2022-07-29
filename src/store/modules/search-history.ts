import { State } from 'vue';
import { ActionContext, Store } from 'vuex';

const KEY = 'search-history';

export interface SearchHistoryState {
    histories: string[];
}

const MutationTypes = {
    put: 'put',
    clear: 'clear'
} as const;

const ActionTypes = {
    put: 'put',
    clear: 'clear'
} as const;

export type SearchHistoryMutationTree = {
    [MutationTypes.put]<T extends string>(state: SearchHistoryState, payload: T): void;
    [MutationTypes.clear](state: SearchHistoryState): void;
}

export type SearchHistoryActionTree = {
    [ActionTypes.put]<T extends string>(this: Store<State>, injectee: ActionContext<SearchHistoryState, State>, payload: T): void;
    [ActionTypes.clear](this: Store<State>, injectee: ActionContext<SearchHistoryState, State>): void;
}

const state: () => SearchHistoryState = () => ({
    histories: ((): string[] => {
        try {
            return JSON.parse(localStorage.getItem(KEY) || "[]");
        } catch (e) {
            return [];
        }
    })()
});

const mutations: SearchHistoryMutationTree = {
    [MutationTypes.put](state, value: string) {
        state.histories = [value, ...state.histories];
    },
    [MutationTypes.clear](state) {
        state.histories = [];
    }
};

const actions: SearchHistoryActionTree = {
    async [ActionTypes.put]({ commit, state }, value) {
        commit(MutationTypes.put, value);
        localStorage.setItem(KEY, JSON.stringify(state.histories));
    },
    async [ActionTypes.clear]({ commit }) {
        commit(MutationTypes.clear);
        localStorage.removeItem(KEY);
    }
};

export default {
    namespaced: true,
    state,
    actions,
    mutations
};
