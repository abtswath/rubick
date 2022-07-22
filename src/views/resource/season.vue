<template>
    <NLayout has-sider>
        <NLayoutSider :width="140">
            <NMenu v-model:value="activated" :options="options" />
        </NLayoutSider>
        <NLayoutContent>
            <Format :data="formats" />
        </NLayoutContent>
    </NLayout>
</template>

<script lang="ts" setup>
import { MenuOption, NLayout, NLayoutSider, NLayoutContent, NMenu } from 'naive-ui';
import { Season } from '@/compositions/use-resource';
import { computed, ref, watchEffect } from 'vue';
import Format from './format.vue';

const props = defineProps<{
    data: Season[]
}>();

const activated = ref(0);

watchEffect(() => {
    if (props.data.length > 0) {
        activated.value = props.data[0].id;
    }
});

const options = computed(() => {
    const options: MenuOption[] = [];
    if (props.data) {
        props.data.forEach(season => {
            options.push({
                key: season.id,
                label: season.name
            });
        });
    }
    return options;
});

const formats = computed(() => {
    const result = props.data.filter(season => season.id === activated.value);
    return result.length > 0 ? result[0].formats : [];
});

</script>

<style lang="scss" scoped>
</style>
