//#region src/preload/preload.js
var { contextBridge, ipcRenderer } = require("electron");
contextBridge.exposeInMainWorld("floatingButtonApi", {
	getConfig: () => ipcRenderer.invoke("floating-button:get-config"),
	onClick: () => ipcRenderer.send("floating-button:clicked"),
	startDrag: () => ipcRenderer.send("floating-button:drag-start"),
	moveDrag: (dx, dy) => ipcRenderer.send("floating-button:drag-move", {
		dx,
		dy
	}),
	endDrag: () => ipcRenderer.send("floating-button:drag-end")
});
contextBridge.exposeInMainWorld("pickCountApi", {
	getConfig: () => ipcRenderer.invoke("pick-count:get-config"),
	cancel: () => ipcRenderer.send("pick-count:cancel"),
	confirm: (count, playMusic) => ipcRenderer.send("pick-count:confirm", {
		count,
		playMusic
	}),
	onOpen: (callback) => {
		const listener = () => callback();
		ipcRenderer.on("pick-count:open", listener);
		return () => {
			ipcRenderer.removeListener("pick-count:open", listener);
		};
	}
});
//#endregion
