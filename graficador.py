import numpy as np
import matplotlib.pyplot as plt
from pathlib import Path

def archivo_texto_a_matriz(ruta_archivo):
    # Abrir el archivo de texto
    path = Path(__file__).parent / ruta_archivo
    with path.open('r') as archivo:
        # Leer líneas del archivo
        lineas = archivo.readlines()

        # Procesar las líneas y crear una matriz
        matriz = [list(map(float, linea.strip().split( ))) for linea in lineas]

        # Convertir la lista de listas a un array de NumPy
        matriz = np.array(matriz)

    return matriz


ruta_archivo = 'potencialrust/output.txt'
phi = archivo_texto_a_matriz(ruta_archivo)


print("Matriz:")
print(phi)


plt.figure(figsize=(8, 6))
#plt.contourf(phi, 100, cmap='inferno')
plt.contour(phi, 8, cmap='inferno')
plt.colorbar(label='Potencial eléctrico [V]')
#plt.quiver(-Ex,-Ey,scale=5)
plt.title('Potencial eléctrico capacitor coplanar basado en curva de Hilbert')
plt.xlabel('x [µm]')
plt.ylabel('y [µm]')
plt.savefig('pothileq.jpg', dpi=500)
plt.show()

plt.figure(figsize=(8, 6))
#plt.contourf(rho, 100, cmap='inferno')
#plt.contour(rho, 8, cmap='inferno')
plt.imshow(phi, cmap='coolwarm', interpolation='nearest')
plt.colorbar(label='Potencial eléctrico [V]')
#plt.quiver(-Ex,-Ey,scale=5)
plt.title('Potencial eléctrico capacitor coplanar basado en curva de Hilbert')
plt.xlabel('x [µm]')
plt.ylabel('y [µm]')
plt.savefig('pothilheat.jpg', dpi=500)
plt.show()

sub_phi = phi[::10, ::10]

# Calcula los gradientes para el subconjunto
Ex, Ey = np.gradient(sub_phi)

# Crea las coordenadas para el quiver plot
x, y = np.meshgrid(np.arange(0, sub_phi.shape[1], 1), np.arange(0, sub_phi.shape[0], 1))

# Grafica el campo eléctrico con quiver
plt.figure(figsize=(8, 6))
plt.streamplot( y, x, -Ey, -Ex,density=3)
plt.axis=("scaled")
plt.title('Campo eléctrico capacitor coplanar basado en curva de Hilbert [V/m]')
plt.xlabel('x [µm]')
plt.ylabel('y [µm]')
plt.savefig('ehilt.jpg', dpi=500)
plt.show()


