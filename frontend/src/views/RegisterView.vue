<template>
  <div class="min-h-screen flex items-center justify-center p-4">
    <AuthForm
      title="Register"
      buttonText="Create Account"
      :loading="loading"
      :errorMessage="apiErrorMessage"
      :successMessage="apiSuccessMessage"
      @submit="handleRegister"
    >
      <InputField
        id="register-username"
        label="Username (Email)"
        type="email"
        placeholder="your@example.com"
        v-model="username"
        :errorMessage="usernameError"
      />
      <InputField
        id="register-password"
        label="Password"
        type="password"
        placeholder="••••••••"
        v-model="password"
        :errorMessage="passwordError"
      />
      <template #footer>
        <router-link
          to="/login"
          class="text-sm text-purple-400 hover:text-purple-300 transition duration-200"
        >
          Already have an account? Login
        </router-link>
      </template>
    </AuthForm>
  </div>
</template>

<script setup>
import { ref, watch } from "vue";
import { useRouter } from "vue-router";
import InputField from "../components/InputField.vue";
import AuthForm from "../components/AuthForm.vue";
import { register } from "../api/auth";

const router = useRouter();

const username = ref("");
const password = ref("");
const usernameError = ref("");
const passwordError = ref("");
const loading = ref(false);
const apiErrorMessage = ref("");
const apiSuccessMessage = ref("");

const validateUsername = () => {
  if (!username.value) {
    usernameError.value = "Username is required.";
  } else if (!/\S+@\S+\.\S+/.test(username.value)) {
    usernameError.value = "Please enter a valid email address.";
  } else {
    usernameError.value = "";
  }
};

const validatePassword = () => {
  if (!password.value) {
    passwordError.value = "Password is required.";
    return;
  }
  const errors = [];
  if (password.value.length < 8) {
    errors.push("at least 8 characters long");
  }
  if (!/[A-Z]/.test(password.value)) {
    errors.push("at least 1 uppercase letter");
  }
  if (!/[a-z]/.test(password.value)) {
    errors.push("at least 1 lowercase letter");
  }
  if (!/[0-9]/.test(password.value)) {
    errors.push("at least 1 numeric digit");
  }
  if (!/[!@#$%^&*()-_=+[{\]}\\|;:'\",<.>/?]/.test(password.value)) {
    errors.push("at least 1 special character (!@#$%^&*)");
  }

  if (errors.length > 0) {
    passwordError.value = `Password must include: ${errors.join(", ")}.`;
  } else {
    passwordError.value = "";
  }
};

watch(username, validateUsername);
watch(password, validatePassword);

const handleRegister = async () => {
  apiErrorMessage.value = "";
  apiSuccessMessage.value = "";
  validateUsername();
  validatePassword();

  if (usernameError.value || passwordError.value) {
    return; 
  }

  loading.value = true;
  const result = await register(username.value, password.value);
  loading.value = false;

  if (result.success) {
    apiSuccessMessage.value = result.message;
    setTimeout(() => {
      router.push("/login");
    }, 1500);
  } else {
    apiErrorMessage.value = result.message;
  }
};
</script>
