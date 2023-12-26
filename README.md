# blivedm_rs

Rust获取bilibili直播弹幕的库，使用WebSocket协议
逻辑参考：https://github.com/xfgryujk/blivedm
需要更改sessdata（从网页上自己cookies里面获取） 及  房间号  （BiliLiveClient::new(sessdata, "5050", tx) 第二个参数就是房间号）  作为练习来写的 自行参考
效果如图：
  
（未实现）配合hudhook项目https://github.com/veeenu/hudhook 可以把这个界面注入dx11,dx12 游戏中方便主播查看
