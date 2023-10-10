# Genshin Artifacter
Rust言語に書き換えられたGenshin Artifacter
# 進化した点
1. 生成速度が速い  
手元だと、rust版は1.633秒、python版は2.844秒でした。  
両方とも50回生成した時の平均時間です。  
(前の結果はエンコーディングやファイル保存の時間を含めていなかったため訂正しました)  
2. メモリに優しい  
rustで書かれているため、メモリの消費が段違いに少ないです。
3. 武器やキャラクターの画像データが自動更新  
頻繁に更新されているデータを取得してくることで、自動更新されています。
# Webサイト
https://artifacter.neody.land/

# Openssl source
https://packages.debian.org/buster/arm64/libssl-dev/download