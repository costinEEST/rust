<script lang="ts">
import { defineComponent, ref, watch } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  name: 'DropDown',
  setup() {
    const options = ref(
      ['Programmer', 'Doctor', 'Dancer'].map(job => ({ text: job, value: job }))
    );
    const selectedOption = ref('');
    const skillRef = ref('')

    watch(selectedOption, (newValue, oldValue) => {
      if (newValue !== oldValue) {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        invoke("skills", { job: selectedOption.value }).then(res => {
          skillRef.value = res as string;
        }).catch(err => console.log(err));
      }
    });

    return { options, selectedOption, skillRef };
  }
});
</script>

<template>
  <select v-model="selectedOption">
    <option disabled value="">Select a job</option>
    <option v-for="option in options" :key="option.value" :value="option.value">
      {{ option.text }}
    </option>
  </select>

  <p>Skills required: <pre>{{ skillRef }}</pre></p>
</template>