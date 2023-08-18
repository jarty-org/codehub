<script setup lang="ts">
const isLogin = ref(false);

const onChangeForm = () => {
  isLogin.value = !isLogin.value;
};
</script>

<template>
  <div class="login-container">
    <div class="form-structor">
      <div class="signup" :class="{ 'slide-up': !isLogin }">
        <h2 @click="onChangeForm" class="form-title" id="signup">
          <span>or</span>Sign up
        </h2>
        <div class="form-holder">
          <input type="text" class="input" placeholder="Username" />
          <input type="email" class="input" placeholder="Email" />
          <input type="password" class="input" placeholder="Password" />
        </div>
        <button class="submit-btn">Sign up</button>
      </div>
      <div class="login" :class="{ 'slide-up': isLogin }">
        <div class="center">
          <h2 @click="onChangeForm" class="form-title" id="login">
            <span>or</span>Log in
          </h2>
          <div class="form-holder">
            <input type="email" class="input" placeholder="Username Or Email" />
            <input type="password" class="input" placeholder="Password" />
          </div>
          <button class="submit-btn">Log in</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="less" scoped>
@import "./index.less";
</style>
