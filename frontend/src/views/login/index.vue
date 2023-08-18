<script setup lang="ts">
import { LoginDTO, SignupDTO, login, signup } from "@/apis/login";

const route = useRouter();
const isLogin = ref(true);

const loginDTO = reactive<LoginDTO>({
  account: "",
  password: "",
});

const signupDTO = reactive<SignupDTO>({
  username: "",
  password: "",
  email: "",
});

const onChangeForm = () => {
  isLogin.value = !isLogin.value;
};

const onLogin = async () => {
  const res = await login(loginDTO);
  if (res) {
    localStorage.setItem("access_token", res.access_token);
    localStorage.setItem("refresh_token", res.refresh_token);
    route.replace("/");
    return;
  }
};

const onSignup = async () => {
  const res = await signup(signupDTO);
  if (res) {
    localStorage.setItem("access_token", res.access_token);
    localStorage.setItem("refresh_token", res.refresh_token);
    route.replace("/");
    return;
  }
};
</script>

<template>
  <div class="login-container">
    <div class="form-structor">
      <div class="login" :class="{ 'slide-up': !isLogin }">
        <h2 @click="onChangeForm" class="form-title" id="login">
          <span>or</span>Log in
        </h2>
        <div class="form-holder">
          <input
            type="text"
            class="input"
            placeholder="Username Or Email"
            v-model="loginDTO.account"
          />
          <input
            type="password"
            class="input"
            placeholder="Password"
            v-model="loginDTO.password"
          />
        </div>
        <button class="submit-btn" @click="onLogin">Log in</button>
      </div>
      <div class="signup" :class="{ 'slide-up': isLogin }">
        <div class="center">
          <h2 @click="onChangeForm" class="form-title" id="signup">
            <span>or</span>Sign up
          </h2>
          <div class="form-holder">
            <input
              type="text"
              class="input"
              placeholder="Username"
              v-model="signupDTO.username"
            />
            <input
              type="email"
              class="input"
              placeholder="Email"
              v-model="signupDTO.email"
            />
            <input
              type="password"
              class="input"
              placeholder="Password"
              v-model="signupDTO.password"
            />
          </div>
          <button class="submit-btn" @click="onSignup">Sign up</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="less" scoped>
@import "./index.less";
</style>
