import { createVNode, render } from "vue";
import Message from "./message.vue";
import { MessageProps } from "./type";

const div = document.createElement("div");
div.setAttribute("class", "j-meassage-container");
document.body.appendChild(div);

export default ({ type, message, duration }: MessageProps) => {
  let timer: number | undefined = undefined;
  const vnode = createVNode(Message, { type, message });
  render(vnode, div);
  clearTimeout(timer);
  timer = setTimeout(() => {
    render(null, div);
  }, duration ?? 3000);
};
