<template>
  <div class="min-h-screen flex items-center justify-center p-4">
    <AuthForm
      title="Login"
      buttonText="Log In"
      :loading="loading"
      :errorMessage="apiErrorMessage"
      :successMessage="apiSuccessMessage"
      @submit="handleLogin"
    >
      <InputField
        id="login-username"
        label="Username (Email)"
        type="email"
        placeholder="your@example.com"
        v-model="username"
        :errorMessage="usernameError"
      />
      <InputField
        id="login-password"
        label="Password"
        type="password"
        placeholder="••••••••"
        v-model="password"
        :errorMessage="passwordError"
      />
      <template #footer>
        <router-link
          to="/register"
          class="text-sm text-purple-400 hover:text-purple-300 transition duration-200"
        >
          Don't have an account? Register
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
import { login } from "../api/auth";

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
  } else {
    passwordError.value = "";
  }
};

watch(username, validateUsername);
watch(password, validatePassword);

const handleLogin = async () => {
  apiErrorMessage.value = "";
  apiSuccessMessage.value = "";
  validateUsername();
  validatePassword();

  if (usernameError.value || passwordError.value) {
    return;
  }

  loading.value = true;
  const result = await login(username.value, password.value);
  loading.value = false;

  if (result.success) {
    localStorage.setItem("userEmail", username.value);
    router.push("/dashboard");
  } else {
    apiErrorMessage.value = result.message;
  }
};
</script>
