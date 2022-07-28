import { RouteRecordRaw } from 'vue-router';
import Layout from '@/components/layout/index.vue';
import BasicLayout from '@/components/basic-layout/index.vue';
import Search from '@/views/search/search.vue';
import Resource from '@/views/resource/index.vue';
import Initialize from '@/views/initialize/index.vue';
import Setting from '@/views/setting/index.vue';
import Favorite from '@/views/favorite/favorite.vue';

export default [
    {
        path: '/:pathMatch(.*)*',
        name: 'NotFound',
        redirect: '/'
    },
    {
        path: '',
        component: Layout,
        children: [
            {
                path: 'initialize',
                name: 'initialize',
                component: Initialize
            },
            {
                path: '',
                component: BasicLayout,
                children: [
                    {
                        path: '',
                        name: 'search',
                        component: Search
                    },
                    {
                        path: 'resource/:id(\\d+)',
                        name: 'resource',
                        component: Resource,
                        props: route => {
                            let id = Number(route.params.id);
                            if (isNaN(id)) {
                                id = 0;
                            }
                            return {
                                id
                            }
                        }
                    },
                    {
                        path: 'setting',
                        name: 'setting',
                        component: Setting
                    },
                    {
                        path: 'favorite',
                        name: 'favorite',
                        component: Favorite
                    }
                ]
            }
        ]
    },
] as RouteRecordRaw[];
