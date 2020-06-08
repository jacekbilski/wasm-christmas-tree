import {draw} from '../build/stage1/feature'

export default class HelloWorld {
    async init(pod) {
        document.getElementById("feature").innerHTML = '<canvas id="canvas"/>';
        await draw();
    }
}
