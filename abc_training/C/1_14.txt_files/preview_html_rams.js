// preview_html_rams.js
//
// Rams document preview support for html-ified documents. Handles coordinating with the parent
// frame to support the floating toolbar and keyboard shortcuts.
//
// Depends on:
// - frame_messenger.js

window.onload = function () {
  PreviewHtml.init();
}

PreviewHtml = {
    // If the mouse hasn't moved after this delay (in msec), consider it idle
    _MOUSEMOVE_IDLE_DELAY: 1500,

    // Keycodes handled by the parent frame's keydown handler
    _keydownKeysHandledByParent: {},

    // Checks if the mouse has been idle after a period of time
    _mousemoveTimeout: null,

    // Used to communicate with parent
    _frameMessenger: null,

    init: function() {
        // Initialize communication with the parent frame
        const FrameMessengerConstructor = FrameMessenger.PreviewFrameMessengerClient || FrameMessenger;
        PreviewHtml._frameMessenger = new FrameMessengerConstructor();
        PreviewHtml._frameMessenger.configureParentMessaging(
            PreviewHtml.trustedMessageFromParentHandler,
            ["print", "enter-fullscreen", "exit-viewer-fullscreen", "clear-mouse-tracking",
             "keydown-keys-handled-by-parent", "disable-download"],
            PreviewHtml.onParentReady
            );
        PreviewHtml._frameMessenger.startListening();

        // Track mouse movements
        document.addEventListener('mousemove', PreviewHtml.onMouseMove);
        document.addEventListener('keydown', PreviewHtml.onKeydown);
    },

    // Called when the parent frame is ready to communicate
    onParentReady: function() {
        PreviewHtml._frameMessenger.postMessageToParent("get-keydown-keys-handled-by-parent");
        PreviewHtml._frameMessenger.postMessageToParent('viewer-ready');
        PreviewHtml._frameMessenger.postMessageToParent("page-change", {
            current_page: 1,
            pages_count: 1,
            doc_type: "html"
        });
    },

    trustedMessageFromParentHandler: function(messageJson) {
        switch (messageJson.action) {
            case "keydown-keys-handled-by-parent":
                PreviewHtml._keydownKeysHandledByParent = messageJson.parameters.keycodes;
                break;
            case "clear-mouse-tracking":
                PreviewHtml._clearMouseTracking();
                break;
            case "enter-fullscreen":
                PreviewHtml.enterFullscreen();
                break;
            case "exit-viewer-fullscreen":
                PreviewHtml.exitFullscreen(false); // Don't notify parent
                break;
            case "print":
                PreviewHtml._print();
                break;
            case "disable-download":
                PreviewHtml.disableDownload();
                break;
        }
    },

    disableDownload: function(){
      document.oncontextmenu = function(event) {event.preventDefault()};
      document.oncopy = function(event) {event.preventDefault()};
    },

    onMouseMove: function(evt) {
      if (PreviewHtml._mousemoveTimeout !== null) {
        // The mouse was moving before, restart the clock
        PreviewHtml._clearMouseTracking();
      }

      PreviewHtml._frameMessenger.postMessageToParent("active-mouse");
      PreviewHtml._mousemoveTimeout = setTimeout(function onIdleMouse() {
        PreviewHtml._frameMessenger.postMessageToParent("idle-mouse");
        PreviewHtml._mousemoveTimeout = null;
      }, PreviewHtml._MOUSEMOVE_IDLE_DELAY);
    },

    onKeydown: function(evt) {
      // TODO(mike): Once KeyboardEvent.key is implemented on all major browsers, we should switch
      // to that. .keyCode is deprecated, but it is the only property supported across all browsers.
      var keycodeStr = evt.keyCode.toString();
      if (Object.keys(PreviewHtml._keydownKeysHandledByParent).indexOf(keycodeStr) >= 0 &&
          !(evt.metaKey && !PreviewHtml._keydownKeysHandledByParent[keycodeStr].metaKey) &&
          !(evt.ctrlKey && !PreviewHtml._keydownKeysHandledByParent[keycodeStr].ctrlKey) &&
          !(evt.altKey  && !PreviewHtml._keydownKeysHandledByParent[keycodeStr].altKey)) {
        PreviewHtml._frameMessenger.postMessageToParent("keydown", {
          keyCode: evt.keyCode,
          ctrlKey: evt.ctrlKey,
          altKey: evt.altKey,
          metaKey: evt.metaKey,
          shiftKey: evt.shiftKey
        });

        var isInlineSearch = evt.keyCode === 70 && (evt.ctrlKey || evt.metaKey);
        if(!isInlineSearch) {
          evt.preventDefault();
        }
      }
    },

    // Stops tracking for idle mouse until next mouse move
    _clearMouseTracking: function() {
      clearTimeout(PreviewHtml._mousemoveTimeout);
      PreviewHtml._mousemoveTimeout = null;
    },

    // Prints the htmlified document
    _print: function() {
        window.print();
    },

    // Enters fullscreen mode
    enterFullscreen: function() {
      if (!document.findElementById("close-x")) {
        // Close "X" isn't in DOM already, create it.
        const closeX = document.createElement("div");
        closeX.id = "close-x";

        const closeXIcon = document.createElement("div");
        closeXIcon.id = "close-x-icon";

        closeX.appendChild(closeXIcon);
        closeX.onclick = PreviewHtml.exitFullscreen;

        document.body.appendChild(closeX);
      }
      document.body.classList.add("fullscreen");
    },

    // Exits fullscreen mode. Defaults to also notifying parent to exit fullscreen.
    exitFullscreen: function (notifyParent) {
      if (typeof notifyParent === "undefined") {
        notifyParent = true;
      }

      document.body.classList.remove("fullscreen");
      if (notifyParent) {
        PreviewHtml._frameMessenger.postMessageToParent("exit-parent-fullscreen");
      }
    },
};
