import { Core } from './src/core/Core';

(() => {
        console.log('Setting up the app');
        const core = new Core();
        core.run();
        console.log('App is ready');
})()