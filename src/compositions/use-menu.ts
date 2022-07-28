import { SearchOutlined, SettingsOutlined, FavoriteBorderOutlined } from '@vicons/material';
import { MenuOption } from 'naive-ui';
import { h, ref } from 'vue';
import { useRouter } from 'vue-router';

export default () => {
    const options: MenuOption[] = [
        {
            icon: () => h(SearchOutlined),
            key: 'search',
            label: '搜索'
        },
        {
            icon: () => h(FavoriteBorderOutlined),
            key: 'favorite',
            label: '收藏'
        }
    ];
    const bottomOptions: MenuOption[] = [
        // {
        //     icon: () => h(SettingsOutlined),
        //     key: 'setting',
        //     label: '设置'
        // }
    ];

    const active = ref(options[0].key);
    const router = useRouter();

    const updateValue = (key: string) => {
        active.value = key;
        router.push({
            name: key
        });
    }

    return {
        options,
        bottomOptions,
        active,
        updateValue
    }
}
