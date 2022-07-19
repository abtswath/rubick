<template>
    <div class="container">
        <NForm ref="formRef" class="search-form" :model="formData" :rules="rules" :show-feedback="false"
            :show-require-mark="false" @submit.prevent="submit" label-placement="left" :label-width="0">
            <NFormItem path="keyword">
                <NInputGroup>
                    <NInput ref="inputRef" v-model:value="formData.keyword" placeholder="搜索剧集" />
                    <NButton attr-type="submit">
                        <template #icon>
                            <NIcon>
                                <SearchOutlined />
                            </NIcon>
                        </template>
                    </NButton>
                </NInputGroup>
            </NFormItem>
        </NForm>
        <History v-if="historyVisible" @select="handleSelect" />
        <template v-else>
            <NSpin :show="loading">
                <Result :resources="resources" />
            </NSpin>
        </template>
    </div>
</template>

<script lang="ts" setup>
import { NForm, NFormItem, NInputGroup, NInput, NButton, NIcon, FormInst, InputInst, NSpin } from 'naive-ui';
import { SearchOutlined } from '@vicons/material';
import { reactive, ref } from 'vue';
import History from './history.vue';
import Result from './result.vue';
import useSearchResources from '@/compositions/use-search-resources';

const formData = reactive({
    keyword: ''
});

const rules = {
    keyword: {
        required: true
    }
};

const formRef = ref<FormInst>();
const inputRef = ref<InputInst>();
const historyVisible = ref(true);

const handleSelect = (item: string) => {
    formData.keyword = item;
    submit();
}

const { loading, search, resources } = useSearchResources();

const submit = () => {
    formRef.value?.validate()
        .then(() => {
            historyVisible.value = false;
            search(formData.keyword);
        })
        .catch(() => {
            inputRef.value?.focus();
        });
};

</script>

<style lang="scss" scoped>
.search-form {
    width: 600px;
    max-width: 80%;
    margin: 0 auto;
    margin-bottom: 8px;
    padding: 16px;
}
.container {
    max-width: 1200px;
    width: 90%;
    margin: 0 auto;
}
</style>
