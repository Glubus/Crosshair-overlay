# 🔊 Dossier Audio - Sons de Clics

Ce dossier contient les fichiers audio pour les sons de clics de souris.

## 📁 Fichiers recommandés

- `click_left.wav` - Son pour le clic gauche
- `click_right.wav` - Son pour le clic droit  
- `click_middle.wav` - Son pour le clic molette

## 🎵 Formats supportés

Pour l'instant, l'application affiche seulement les informations des clics dans la console.
L'implémentation audio complète sera ajoutée dans une version future.

## 💡 Comment obtenir des sons

1. **Sites gratuits :**
   - Freesound.org
   - Zapsplat.com
   - Mixkit.co

2. **Formats recommandés :**
   - WAV (meilleure qualité)
   - MP3 (plus compact)
   - OGG (bon compromis)

3. **Caractéristiques recommandées :**
   - Durée : 0.1-0.5 secondes
   - Volume modéré
   - Sons courts et nets

## ⚙️ Configuration

Modifiez la section `[audio]` dans `config.toml` pour :
- Activer/désactiver l'audio global
- Choisir quels boutons déclenchent des sons
- Ajuster le volume et la hauteur (pitch)
- Spécifier les fichiers audio

## 🚀 Activation

1. Placez vos fichiers audio dans ce dossier
2. Activez l'audio dans config.toml : `enabled = true`
3. Configurez les sons pour chaque bouton
4. Relancez l'application ou appuyez sur F5

## 🔧 Exemple de configuration

```toml
[audio]
enabled = true
volume = 0.7

[audio.left_click]
enabled = true
sound_file = "sounds/click_left.wav"
volume = 1.0
pitch = 1.0

[audio.right_click]
enabled = true
sound_file = "sounds/click_right.wav"
volume = 0.8
pitch = 1.2
``` 