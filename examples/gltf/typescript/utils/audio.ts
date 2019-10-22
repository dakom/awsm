let _ctx:AudioContext;

export const get_audio_context = () => {
    if(!_ctx) {
        const ctor = (window as any).AudioContext || (window as any).webkitAudioContext;
        _ctx = new ctor();
    }

    return _ctx;
}