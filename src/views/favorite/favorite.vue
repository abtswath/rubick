<template>
    <NSpin :show="loading">
        <NEmpty v-if="!loading && resources.length <= 0" description="你什么也找不到" />
        <NSpace>
            <template v-for="resource in resources">
                <RouterLink :to="{ name: 'resource', params: { id: resource.id } }">
                    <NCard size="small" :style="{ width: '160px' }" :title="resource.name">
                        <template #cover>
                            <div class="cover" :style="{ backgroundImage: `url(${resource.pic})` }" />
                        </template>
                        {{ resource.original_name }}
                    </NCard>
                </RouterLink>
            </template>
        </NSpace>
    </NSpin>
</template>

<script lang="ts" setup>
import { NSpin, NCard, NSpace, NEmpty } from 'naive-ui';
import useFavorites from '@/compositions/use-favorites';

const { loading, resources } = useFavorites();

</script>

<style lang="scss" scoped>
.cover {
    height: 210px;
    background-size: cover;
    background-position: center center;
    background-repeat: no-repeat;
    width: 100%;
}
</style>
