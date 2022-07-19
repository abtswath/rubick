<template>
    <router-link class="show" :to="{ name: 'resource', params: { id: resource.id } }">
        <NThing>
            <template #avatar>
                <NAvatar size="large" round
                    :color="resource.channel ? stringToColor(resource.channel) : '#ccc'">
                    {{ resource.channel }}
                </NAvatar>
            </template>
            <template #header>
                <div class="ellipsis">{{ resource.name }}</div>
            </template>
            <template #description>
                <div class="ellipsis">
                    {{ [resource.original_name, resource.alias_name].filter(item => item !== '').join('/') }}
                </div>
            </template>
        </NThing>
    </router-link>
</template>

<script lang="ts" setup>
import { NThing, NAvatar } from 'naive-ui';
import { SearchResult } from '@/compositions/use-search-resources';

defineProps<{
    resource: SearchResult
}>();

const stringToColor = (str: string) => {
    let hash = 0;
    let i;
    for (i = 0; i < str.length; i += 1) {
        hash = str.charCodeAt(i) + ((hash << 5) - hash);
    }

    let color = '#';

    for (i = 0; i < 3; i += 1) {
        const value = (hash >> (i * 8)) & 0xff;
        color += `00${value.toString(16)}`.slice(-2);
    }

    return color;
}
</script>

<style lang="scss" scoped>
</style>
