# ネットワークを再定義
sudo virsh net-define xml/for-wg2-nw.xml
# ネットワークを起動
sudo virsh net-start for-wg2
# ネットワークを終了
sudo virsh net-destroy for-wg2
# ネットワークの定義を削除
sudo virsh net-undefine for-wg2

# vm1.xmlを定義
sudo virsh define xml/vm1.xml
# vm1を起動 
sudo virsh start wg2-vm1
# vm1を終了
sudo virsh destroy wg2-vm1
# vm1の定義を削除
sudo virsh undefine wg2-vm1
