<template>
  <div class="mb-4">
    <label :for="id" class="block text-sm font-medium text-gray-300 mb-1">
      {{ label }}
    </label>
    <input
      :id="id"
      :type="type"
      :value="modelValue"
      @input="$emit('update:modelValue', $event.target.value)"
      :placeholder="placeholder"
      class="w-full px-4 py-2 bg-gray-700 text-gray-100 border border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent transition duration-200 ease-in-out"
      :class="{ 'border-red-500': errorMessage }"
    />
    <p v-if="errorMessage" class="mt-1 text-sm text-red-400">
      {{ errorMessage }}
    </p>
  </div>
</template>

<script setup>
import { defineProps, defineEmits } from "vue";

defineProps({
  id: {
    type: String,
    required: true,
  },
  label: {
    type: String,
    required: true,
  },
  type: {
    type: String,
    default: "text",
  },
  modelValue: {
    type: String,
    default: "",
  },
  placeholder: {
    type: String,
    default: "",
  },
  errorMessage: {
    type: String,
    default: "",
  },
});

defineEmits(["update:modelValue"]);
</script>
