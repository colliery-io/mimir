// Composable for loading book images

import { invoke } from '@tauri-apps/api/core'

export function useImageLoader() {
  
  // Load image for a book
  async function loadBookImage(bookId: string, imagePath: string, imageId: string) {
    console.log('Loading image:', { bookId, imagePath, imageId })
    
    setTimeout(async () => {
      const imgElement = document.getElementById(imageId)
      if (imgElement && bookId) {
        try {
          const response = await invoke<{ success: boolean; data?: string; message?: string }>('serve_book_image', {
            bookId: bookId,
            imagePath: imagePath
          })
          
          console.log('Image response:', response)
          
          if (response.success && response.data) {
            const imageName = imagePath.split('/').pop() || 'image'
            imgElement.innerHTML = `<img src="${response.data}" alt="${imageName}" style="max-width: 100%; height: auto; display: block; margin: 0 auto;" />`
            // Remove the min-height style 
            imgElement.style.minHeight = ''
          } else {
            console.error('Failed to load image - no data in response')
            imgElement.innerHTML = `<div class="image-error">Failed to load image</div>`
          }
        } catch (error) {
          console.error('Failed to load image:', error)
          imgElement.innerHTML = `<div class="image-error">Failed to load image</div>`
        }
      } else {
        console.error('Missing element or bookId:', { imgElement, bookId })
      }
    }, 0)
  }
  
  return {
    loadBookImage
  }
}