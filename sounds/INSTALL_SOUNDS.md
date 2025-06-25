# 🔊 Installation des Sons

Pour tester l'audio, vous devez ajouter des fichiers audio dans ce dossier.

## 📁 Fichiers requis (selon votre config.toml)

- `click_left.wav` - Son pour clic gauche
- `click_right.wav` - Son pour clic droit
- `click_middle.wav` - Son pour clic molette

## 🎵 Formats supportés par Rodio

- **WAV** (recommandé)
- **MP3** 
- **FLAC**
- **OGG**

## 🚀 Test rapide

1. **Téléchargez des sons gratuits :**
   - https://freesound.org/search/?q=click
   - https://mixkit.co/free-sound-effects/click/
   - https://pixabay.com/sound-effects/search/click/

2. **Ou créez des sons simples :**
   - Enregistrez un clic avec votre téléphone
   - Utilisez Audacity (gratuit) pour créer des bips courts
   - Convertissez en WAV

3. **Nommez les fichiers :**
   - `click_left.wav`
   - `click_right.wav` 
   - `click_middle.wav`

4. **Placez-les dans ce dossier `sounds/`**

5. **Activez l'audio dans config.toml :**
   ```toml
   [audio]
   enabled = true
   ```

6. **Lancez l'application et cliquez !**

## 🔧 Dépannage

- **"Fichier audio introuvable"** → Vérifiez le nom et l'emplacement
- **"Erreur lecture audio"** → Vérifiez le format du fichier
- **"Système audio non initialisé"** → Problème avec votre carte son

## 💡 Conseils

- Utilisez des sons courts (< 0.5 seconde)
- Volume modéré pour éviter les sursauts
- Testez différents pitch pour varier les sons 